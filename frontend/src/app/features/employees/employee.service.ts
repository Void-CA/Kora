import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';

import { API_BASE } from '../../api';

const EMPLOYEES = `${API_BASE}/employees`;

export interface Employee {
  id: string;
  name: string;
  active: boolean;
}

export interface CreateEmployeeRequest {
  name: string;
}

@Injectable({ providedIn: 'root' })
export class EmployeeService {
  private readonly http = inject(HttpClient);

  list() {
    return this.http.get<Employee[]>(EMPLOYEES);
  }

  create(body: CreateEmployeeRequest) {
    return this.http.post<Employee>(EMPLOYEES, body);
  }
}
