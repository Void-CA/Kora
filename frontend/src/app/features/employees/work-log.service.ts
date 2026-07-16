import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';

import { API_BASE } from '../../api';

const WORK_LOGS = `${API_BASE}/work-logs`;

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

  listByEmployee(employeeId: string) {
    return this.http.get<WorkLog[]>(`${API_BASE}/employees/${employeeId}/work-logs`);
  }

  create(body: CreateWorkLogRequest) {
    return this.http.post<WorkLog>(WORK_LOGS, body);
  }
}
