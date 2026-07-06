import { Component, input } from '@angular/core';

@Component({
  selector: 'button[ui-button]',
  template: '<ng-content />',
  host: {
    '[attr.disabled]': 'disabled() ? true : null',
    '[attr.aria-disabled]': 'disabled()',
  },
  styleUrl: './button.component.scss',
})
export class UiButton {
  readonly disabled = input(false);
}
