import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';

import { CreateWorkLogRequest } from '../work-log.service';

@Component({
  selector: 'app-work-log-form-dialog',
  imports: [MatDialogModule, MatFormFieldModule, MatInputModule, MatButtonModule, FormsModule],
  template: `
    <h2 mat-dialog-title>Register Work Hours</h2>

    <mat-dialog-content>
      <mat-form-field class="w-full" appearance="fill">
        <mat-label>Date</mat-label>
        <input matInput [(ngModel)]="workedOn" type="date" />
      </mat-form-field>

      <mat-form-field class="w-full" appearance="fill">
        <mat-label>Hours</mat-label>
        <input matInput [(ngModel)]="hours" type="number" step="0.5" min="0" placeholder="8" />
      </mat-form-field>
    </mat-dialog-content>

    <mat-dialog-actions align="end">
      <button mat-button mat-dialog-close>Cancel</button>
      <button
        mat-raised-button
        color="primary"
        [disabled]="!workedOn || !hours || hours <= 0"
        (click)="save()"
      >
        Save
      </button>
    </mat-dialog-actions>
  `,
})
export class WorkLogFormDialogComponent {
  private readonly dialogRef = inject(MatDialogRef<WorkLogFormDialogComponent>);

  protected workedOn = '';
  protected hours = 0;

  save() {
    if (!this.workedOn || this.hours <= 0) return;
    this.dialogRef.close({
      employee_id: '', // filled by parent
      worked_on: this.workedOn,
      hours: this.hours,
    } satisfies CreateWorkLogRequest);
  }
}
