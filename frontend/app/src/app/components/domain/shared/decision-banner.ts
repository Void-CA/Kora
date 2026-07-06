import { Component, input } from '@angular/core';

@Component({
  selector: 'kora-decision-banner',
  template: `
    <div class="banner" [class.banner--high]="priority()==='high'"
          [class.banner--medium]="priority()==='medium'">
      <div class="banner__body">
        <span class="banner__tag">{{ tag() }}</span>
        <p class="banner__title">{{ title() }}</p>
        @if (reason()) { <p class="banner__reason">{{ reason() }}</p> }
      </div>
      @if (actionLabel()) {
        <button class="banner__action">{{ actionLabel() }}</button>
      }
    </div>
  `,
  styleUrl: './decision-banner.component.scss',
})
export class DecisionBanner {
  readonly tag = input.required<string>();
  readonly title = input.required<string>();
  readonly reason = input<string>('');
  readonly actionLabel = input<string>('');
  readonly priority = input<'low' | 'medium' | 'high'>('medium');
}
