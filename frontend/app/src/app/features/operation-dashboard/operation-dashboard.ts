import { Component, signal, inject } from '@angular/core';
import { StatusSummary, StatusCounts } from './components/status-summary';
import { NextActionCard, NextAction } from './components/next-action-card';
import { AttentionList, AttentionItem } from './components/attention-list';
import { getOperationToday, OperationToday as ApiOperationToday } from '../../api/kora-api';

@Component({
  selector: 'app-operation-dashboard',
  imports: [StatusSummary, NextActionCard, AttentionList],
  templateUrl: './operation-dashboard.html',
  styleUrl: './operation-dashboard.scss',
})
export class OperationDashboard {
  readonly state = signal<ApiOperationToday | null>(null);
  readonly error = signal<string | null>(null);

  constructor() {
    getOperationToday()
      .then(data => this.state.set(data))
      .catch(err => this.error.set(String(err)));
  }
}
