import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';

export interface WorkLog {
  id: string;
  employee_id: string;
  worked_on: string;
  hours: number;
}

export interface CreateWorkLogRequest {
  employee_id: string;
  worked_on: string;
  hours: number;
}

@Injectable({ providedIn: 'root' })
export class WorkLogService {
  private readonly http = inject(HttpClient);
  private readonly baseUrl = '/api/v1/work-logs';

  listByEmployee(employeeId: string) {
    return this.http.get<WorkLog[]>(`/api/v1/employees/${employeeId}/work-logs`);
  }

  create(body: CreateWorkLogRequest) {
    return this.http.post<WorkLog>(this.baseUrl, body);
  }
}
