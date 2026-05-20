'use client';

import { useEffect } from 'react';

type RenderReadyEmitterProps = {
  draftId: string;
};

export default function RenderReadyEmitter({ draftId }: RenderReadyEmitterProps) {
  useEffect(() => {
    const detail = { draftId };
    window.dispatchEvent(new CustomEvent('render-ready', { detail }));

    import('@tauri-apps/api/event')
      .then(({ emit }) => emit('render-ready', detail))
      .catch(() => {
        // Browser builds do not have a Tauri event bridge available.
      });
  }, [draftId]);

  return null;
}
