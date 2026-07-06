// Infrastructure — thin wrapper, no Kora identity.
// Handles focus, disabled, aria, keyboard navigation.
// Can be swapped or upgraded without touching the rest of Kora.

import { Component, input, output } from '@angular/core';

@Component({
  selector: 'button[ui-button]',
  template: '<ng-content />',
  host: {
    '[attr.disabled]': 'disabled() ? true : null',
    '[attr.aria-disabled]': 'disabled()',
  },
  styles: [`
    :host {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 0.5rem;
      font-family: inherit;
      font-size: inherit;
      line-height: 1;
      cursor: pointer;
      border: none;
      background: none;
      padding: 0;
      transition: opacity 0.12s;
    }
    :host:focus-visible {
      outline: 2px solid currentColor;
      outline-offset: 2px;
    }
    :host[disabled] {
      opacity: 0.4;
      pointer-events: none;
    }
  `],
})
export class UiButton {
  readonly disabled = input(false);
}
