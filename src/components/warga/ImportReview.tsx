import type { ImportPreviewRow } from '@/types/warga';

interface ImportReviewProps {
  rows: ImportPreviewRow[];
}

export default function ImportReview({ rows }: ImportReviewProps) {
  const validCount = rows.filter((r) => r.status === 'valid').length;
  const errorCount = rows.filter((r) => r.status === 'error').length;
  const conflictCount = rows.filter((r) => r.status === 'conflict').length;

  return (
    <div className="space-y-4">
      <div className="stats shadow">
        <div className="stat">
          <div className="stat-title">Valid</div>
          <div className="stat-value text-success">{validCount}</div>
        </div>
        <div className="stat">
          <div className="stat-title">Errors</div>
          <div className="stat-value text-error">{errorCount}</div>
        </div>
        <div className="stat">
          <div className="stat-title">Conflicts</div>
          <div className="stat-value text-warning">{conflictCount}</div>
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="table table-zebra">
          <thead>
            <tr>
              <th>Row</th>
              <th>NIK</th>
              <th>No KK</th>
              <th>Nama</th>
              <th>Status</th>
              <th>Message</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.rowNumber}>
                <td>{row.rowNumber}</td>
                <td className="font-mono text-sm">{row.nik}</td>
                <td className="font-mono text-sm">{row.noKk}</td>
                <td>{row.namaLengkap}</td>
                <td>
                  <span
                    className={`badge ${
                      row.status === 'valid'
                        ? 'badge-success'
                        : row.status === 'error'
                          ? 'badge-error'
                          : 'badge-warning'
                    }`}
                  >
                    {row.status}
                  </span>
                </td>
                <td className="text-sm">{row.message || '-'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
