import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';
import type { FieldsView } from '../../core/view-models';
import { MOCK_FIELDS, mockDelay } from '../../core/mock-data';

@Component({
  selector: 'app-fields',
  imports: [RouterLink],
  templateUrl: './fields-page.component.html',
  styleUrl: './fields-page.component.scss',
})
export class FieldsPage {
  readonly vm = signal<FieldsView | null>(null);
  constructor() { mockDelay().then(() => this.vm.set(MOCK_FIELDS)); }
}
