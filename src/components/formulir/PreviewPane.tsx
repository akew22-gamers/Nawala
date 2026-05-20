/**
 * PreviewPane - Sandboxed iframe for form preview
 */

interface PreviewPaneProps {
  html: string;
}

export function PreviewPane({ html }: PreviewPaneProps) {
  return (
    <iframe
      className="h-[80vh] w-full rounded-box border border-base-300 bg-white"
      sandbox="allow-same-origin"
      srcDoc={html}
      title="Preview formulir"
    />
  );
}
