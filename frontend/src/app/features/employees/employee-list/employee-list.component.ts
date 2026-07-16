import { Component, inject, OnInit } from '@angular/core';
import { DatePipe } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatDialog } from '@angular/material/dialog';
import { MatIconModule } from '@angular/material/icon';
import { MatTableModule } from '@angular/material/table';
import { MatToolbarModule } from '@angular/material/toolbar';

import { EmployeeService } from '../employee.service';
import { WorkLogService } from '../work-log.service';
import { EmployeeFormDialogComponent } from '../employee-form/employee-form-dialog.component';
import { WorkLogFormDialogComponent } from '../work-log-form/work-log-form-dialog.component';

@Component({
  selector: 'app-employee-list',
  imports: [
    MatToolbarModule,
    MatButtonModule,
    MatTableModule,
    MatIconModule,
    DatePipe,
  ],
  template: `
    <mat-toolbar color="primary">
      <span>Kora — Employees</span>
      <span class="flex-1"></span>
      <button mat-raised-button color="accent" (click)="addEmployee()">
        Add Employee
      </button>
    </mat-toolbar>

    <div class="p-4">
      <table mat-table [dataSource]="employees" class="w-full">
        <ng-container matColumnDef="name">
          <th mat-header-cell *matHeaderCellDef>Name</th>
          <td mat-cell *matCellDef="let e">{{ e.name }}</td>
        </ng-container>

        <ng-container matColumnDef="active">
          <th mat-header-cell *matHeaderCellDef>Active</th>
          <td mat-cell *matCellDef="let e">{{ e.active ? 'Yes' : 'No' }}</td>
        </ng-container>

        <ng-container matColumnDef="actions">
          <th mat-header-cell *matHeaderCellDef></th>
          <td mat-cell *matCellDef="let e">
            <button mat-icon-button (click)="logHours(e)">
              <mat-icon>add_circle</mat-icon>
            </button>
            <button mat-icon-button (click)="selectedEmployee = e; loadLogs()">
              <mat-icon>list_alt</mat-icon>
            </button>
          </td>
        </ng-container>

        <tr mat-header-row *matHeaderRowDef="columns"></tr>
        <tr mat-row *matRowDef="let row; columns: columns"></tr>
      </table>

      @if (selectedEmployee) {
        <div class="mt-6 border-t pt-4">
          <div class="flex items-center justify-between mb-4">
            <h2 class="text-xl font-semibold">
              Work Logs — {{ selectedEmployee.name }}
            </h2>
            <button mat-stroked-button (click)="logHours(selectedEmployee)">
              <mat-icon class="mr-1">add</mat-icon>
              Log Hours
            </button>
          </div>

          @if (workLogs.length === 0) {
            <p class="text-gray-500">No work logs yet.</p>
          } @else {
            <table mat-table [dataSource]="workLogs" class="w-full">
              <ng-container matColumnDef="worked_on">
                <th mat-header-cell *matHeaderCellDef>Date</th>
                <td mat-cell *matCellDef="let w">{{ w.worked_on | date }}</td>
              </ng-container>

              <ng-container matColumnDef="hours">
                <th mat-header-cell *matHeaderCellDef>Hours</th>
                <td mat-cell *matCellDef="let w">{{ w.hours }}</td>
              </ng-container>

              <tr mat-header-row *matHeaderRowDef="logColumns"></tr>
              <tr mat-row *matRowDef="let row; columns: logColumns"></tr>
            </table>
          }
        </div>
      }
    </div>
  `,
})
export class EmployeeListComponent implements OnInit {
  private readonly employeeService = inject(EmployeeService);
  private readonly workLogService = inject(WorkLogService);
  private readonly dialog = inject(MatDialog);

  protected readonly columns = ['name', 'active', 'actions'];
  protected readonly logColumns = ['worked_on', 'hours'];

  protected employees: import('../employee.service').Employee[] = [];
  protected workLogs: import('../work-log.service').WorkLog[] = [];
  protected selectedEmployee: import('../employee.service').Employee | null = null;

  ngOnInit() {
    this.loadEmployees();
  }

  private loadEmployees() {
    this.employeeService.list().subscribe((data) => (this.employees = data));
  }

  protected loadLogs() {
    if (!this.selectedEmployee) return;
    this.workLogService
      .listByEmployee(this.selectedEmployee.id)
      .subscribe((data) => (this.workLogs = data));
  }

  protected addEmployee() {
    const ref = this.dialog.open(EmployeeFormDialogComponent);
    ref.afterClosed().subscribe((result) => {
      if (result) {
        this.employeeService.create(result).subscribe(() => this.loadEmployees());
      }
    });
  }

  protected logHours(employee: import('../employee.service').Employee) {
    const ref = this.dialog.open(WorkLogFormDialogComponent);
    ref.afterClosed().subscribe((result) => {
      if (result) {
        result.employee_id = employee.id;
        this.workLogService.create(result).subscribe(() => {
          this.selectedEmployee = employee;
          this.loadLogs();
          this.loadEmployees();
        });
      }
    });
  }
}
