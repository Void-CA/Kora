import { Component, signal } from '@angular/core';
import { StatusSummary } from './components/status-summary';
import { NextActionCard } from './components/next-action-card';
import { AttentionList } from './components/attention-list';
import { getOperationToday, OperationToday } from '../../api/kora-api';

@Component({
  selector: 'app-operation-dashboard',
  imports: [StatusSummary, NextActionCard, AttentionList],
  templateUrl: './operation-dashboard.html',
  styleUrl: './operation-dashboard.scss',
})
export class OperationDashboard {
  readonly state = signal<OperationToday | null>(null);
  readonly error = signal<string | null>(null);

  constructor() {
    getOperationToday()
      .then(data => this.state.set(data))
      .catch(err => this.error.set(String(err)));
  }
}
