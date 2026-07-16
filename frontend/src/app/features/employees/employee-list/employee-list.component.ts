import { Component, inject, OnInit } from '@angular/core';
import { DatePipe } from '@angular/common';
import { MatTableModule } from '@angular/material/table';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';

import { EmployeeService } from '../employee.service';

@Component({
  selector: 'app-employee-list',
  imports: [MatTableModule, MatToolbarModule, MatButtonModule],
  template: `
    <mat-toolbar color="primary">
      <span>Kora — Employees</span>
    </mat-toolbar>

    <div class="p-4">
      <div class="flex justify-end mb-4">
        <button mat-raised-button color="primary">Add Employee</button>
      </div>

      <table mat-table [dataSource]="employees" class="w-full">
        <ng-container matColumnDef="name">
          <th mat-header-cell *matHeaderCellDef>Name</th>
          <td mat-cell *matCellDef="let e">{{ e.name }}</td>
        </ng-container>

        <ng-container matColumnDef="active">
          <th mat-header-cell *matHeaderCellDef>Active</th>
          <td mat-cell *matCellDef="let e">{{ e.active ? 'Yes' : 'No' }}</td>
        </ng-container>

        <tr mat-header-row *matHeaderRowDef="columns"></tr>
        <tr mat-row *matRowDef="let row; columns: columns"></tr>
      </table>
    </div>
  `,
})
export class EmployeeListComponent implements OnInit {
  private readonly service = inject(EmployeeService);

  protected readonly columns = ['name', 'active'];
  protected employees: import('../employee.service').Employee[] = [];

  ngOnInit() {
    this.service.list().subscribe((data) => (this.employees = data));
  }
}
