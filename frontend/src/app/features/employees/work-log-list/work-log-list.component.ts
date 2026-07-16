import { Component, inject, input, OnInit } from '@angular/core';
import { DatePipe } from '@angular/common';
import { MatTableModule } from '@angular/material/table';

import { WorkLogService } from '../work-log.service';

@Component({
  selector: 'app-work-log-list',
  imports: [MatTableModule, DatePipe],
  template: `
    <h3 class="text-lg font-medium mb-2">Work Logs</h3>

    @if (logs.length === 0) {
      <p class="text-gray-500">No work logs yet.</p>
    } @else {
      <table mat-table [dataSource]="logs" class="w-full">
        <ng-container matColumnDef="worked_on">
          <th mat-header-cell *matHeaderCellDef>Date</th>
          <td mat-cell *matCellDef="let w">{{ w.worked_on | date }}</td>
        </ng-container>

        <ng-container matColumnDef="hours">
          <th mat-header-cell *matHeaderCellDef>Hours</th>
          <td mat-cell *matCellDef="let w">{{ w.hours }}</td>
        </ng-container>

        <tr mat-header-row *matHeaderRowDef="columns"></tr>
        <tr mat-row *matRowDef="let row; columns: columns"></tr>
      </table>
    }
  `,
})
export class WorkLogListComponent implements OnInit {
  private readonly service = inject(WorkLogService);

  readonly employeeId = input.required<string>();
  protected readonly columns = ['worked_on', 'hours'];
  protected logs: import('../work-log.service').WorkLog[] = [];

  ngOnInit() {
    this.service.listByEmployee(this.employeeId()).subscribe((data) => (this.logs = data));
  }
}
