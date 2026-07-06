import { Component, input } from '@angular/core';
import { UiButton } from '../ui/button';

export type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'danger';

@Component({
  selector: 'kora-button',
  imports: [UiButton],
  template: `
    <button ui-button [disabled]="disabled()" [class]="'btn btn--' + variant()">
      <ng-content />
    </button>
  `,
  styleUrl: './button.component.scss',
})
export class KoraButton {
  readonly disabled = input(false);
  readonly variant = input<ButtonVariant>('primary');
}
