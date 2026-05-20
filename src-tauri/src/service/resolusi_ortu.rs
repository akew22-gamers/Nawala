use strsim::jaro_winkler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionStatus {
    Auto,
    Confirm,
    Manual,
}

#[derive(Debug, Clone)]
pub struct FamilyMember {
    pub nik: String,
    pub nama: String,
    pub jenis_kelamin: String,
    pub hubungan_keluarga: String,
}

#[derive(Debug, Clone)]
pub struct ParentResolution {
    pub nik: Option<String>,
    pub status: ResolutionStatus,
    pub score: f64,
    pub matched_name: Option<String>,
}

pub fn resolve_parent(
    target_name: &str,
    members: &[FamilyMember],
    allowed_relations: &[&str],
    expected_gender: &str,
) -> ParentResolution {
    let target_lower = target_name.trim().to_lowercase();
    
    // Filter candidates by gender and allowed relations
    let mut candidates: Vec<(&FamilyMember, f64)> = members
        .iter()
        .filter(|m| {
            let gender_match = m.jenis_kelamin.trim().eq_ignore_ascii_case(expected_gender);
            let relation_match = allowed_relations
                .iter()
                .any(|r| r.eq_ignore_ascii_case(m.hubungan_keluarga.trim()));
            gender_match && relation_match
        })
        .map(|m| {
            let score = jaro_winkler(&target_lower, &m.nama.trim().to_lowercase());
            (m, score)
        })
        .collect();
    
    // Sort by score descending
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    // Apply thresholds
    if let Some((best_member, best_score)) = candidates.first() {
        if *best_score >= 0.90 {
            return ParentResolution {
                nik: Some(best_member.nik.clone()),
                status: ResolutionStatus::Auto,
                score: *best_score,
                matched_name: Some(best_member.nama.clone()),
            };
        } else if *best_score >= 0.70 {
            return ParentResolution {
                nik: Some(best_member.nik.clone()),
                status: ResolutionStatus::Confirm,
                score: *best_score,
                matched_name: Some(best_member.nama.clone()),
            };
        }
    }
    
    // No good match
    ParentResolution {
        nik: None,
        status: ResolutionStatus::Manual,
        score: 0.0,
        matched_name: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_father_inside_same_kk() {
        let members = vec![FamilyMember {
            nik: "3210221506570081".to_string(),
            nama: "SENA".to_string(),
            jenis_kelamin: "L".to_string(),
            hubungan_keluarga: "Kepala Keluarga".to_string(),
        }];
        let result = resolve_parent("SENA", &members, &["Kepala Keluarga", "Suami", "Ayah"], "L");
        assert_eq!(result.nik.as_deref(), Some("3210221506570081"));
        assert_eq!(result.status, ResolutionStatus::Auto);
    }
}
