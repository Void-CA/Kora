import { Component, input } from '@angular/core';

@Component({
  selector: 'kora-section',
  template: `
    <section class="section">
      @if (label()) {
        <h2 class="section__label">{{ label() }}</h2>
      }
      <ng-content />
    </section>
  `,
  styleUrl: './section.component.scss',
})
export class KoraSection {
  readonly label = input<string>('');
}
