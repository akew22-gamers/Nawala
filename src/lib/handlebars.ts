import Handlebars from 'handlebars';

/**
 * Register custom Handlebars helper for Indonesian date formatting
 */
Handlebars.registerHelper('tgl_indo', (value: string) => {
  const date = new Date(`${value}T00:00:00Z`);
  return new Intl.DateTimeFormat('id-ID', {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
    timeZone: 'UTC',
  }).format(date);
});

/**
 * Render a Handlebars template with the given context
 * @param template - Handlebars template string
 * @param context - Data context for template rendering
 * @returns Rendered HTML string
 */
export function renderTemplate(template: string, context: unknown): string {
  return Handlebars.compile(template)(context);
}
