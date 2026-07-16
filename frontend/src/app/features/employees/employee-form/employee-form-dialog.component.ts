import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';

import { CreateEmployeeRequest } from '../employee.service';

@Component({
  selector: 'app-employee-form-dialog',
  imports: [MatDialogModule, MatFormFieldModule, MatInputModule, MatButtonModule, FormsModule],
  template: `
    <h2 mat-dialog-title>New Employee</h2>

    <mat-dialog-content>
      <mat-form-field class="w-full" appearance="fill">
        <mat-label>Name</mat-label>
        <input matInput [(ngModel)]="name" placeholder="Employee name" />
      </mat-form-field>
    </mat-dialog-content>

    <mat-dialog-actions align="end">
      <button mat-button mat-dialog-close>Cancel</button>
      <button mat-raised-button color="primary" [disabled]="!name.trim()" (click)="save()">
        Save
      </button>
    </mat-dialog-actions>
  `,
})
export class EmployeeFormDialogComponent {
  private readonly dialogRef = inject(MatDialogRef<EmployeeFormDialogComponent>);

  protected name = '';

  save() {
    if (!this.name.trim()) return;
    this.dialogRef.close({ name: this.name.trim() } satisfies CreateEmployeeRequest);
  }
}
