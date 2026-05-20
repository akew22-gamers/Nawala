import RenderReadyEmitter from './RenderReadyEmitter';

type PrintPageProps = {
  params: Promise<{ draftId: string }>;
};

export function generateStaticParams() {
  return [{ draftId: 'sample-draft' }];
}

export default async function PrintPage({ params }: PrintPageProps) {
  const { draftId } = await params;

  return (
    <main className="min-h-screen bg-neutral-200 px-4 py-8 text-neutral-950 print:bg-white print:p-0">
      <RenderReadyEmitter draftId={draftId} />
      <style>{`
        @page { size: A4; margin: 18mm 16mm; }
        @media print {
          body { background: white !important; }
          .print-sheet { box-shadow: none !important; margin: 0 !important; width: auto !important; min-height: auto !important; }
          .no-print { display: none !important; }
        }
      `}</style>

      <section className="print-sheet mx-auto min-h-[297mm] w-full max-w-[210mm] bg-white p-10 shadow-xl print:p-0">
        <header className="border-neutral-900 border-b-4 pb-4 text-center">
          <p className="font-semibold text-sm uppercase tracking-[0.35em]">Pemerintah Desa</p>
          <h1 className="mt-2 font-bold text-3xl uppercase">Dokumen Surat</h1>
          <p className="mt-1 text-sm">Pratinjau cetak untuk draft {draftId}</p>
        </header>

        <article className="mt-10 space-y-6 text-base leading-8">
          <p className="text-right">Nomor Draft: {draftId}</p>
          <p>
            Yang bertanda tangan di bawah ini menerangkan bahwa dokumen ini adalah placeholder
            pratinjau cetak.
          </p>
          <p>
            Konten final akan dimuat dari riwayat atau draft formulir setelah pipeline render HTML
            surat tersedia. Halaman ini sudah siap dipakai sebagai target webview untuk proses
            ekspor PDF berikutnya.
          </p>
          <div className="mt-12 grid grid-cols-2 gap-8 text-center">
            <div>
              <p>Pemohon</p>
              <div className="h-24" />
              <p className="font-semibold underline">Nama Pemohon</p>
            </div>
            <div>
              <p>Pejabat Desa</p>
              <div className="h-24" />
              <p className="font-semibold underline">Nama Pejabat</p>
            </div>
          </div>
        </article>

        <footer className="no-print mt-10 rounded-lg border border-amber-300 bg-amber-50 p-4 text-amber-900 text-sm">
          Placeholder cetak: data asli draft belum dihubungkan pada Task 9.
        </footer>
      </section>
    </main>
  );
}
