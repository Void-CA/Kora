import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';

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
  private readonly baseUrl = '/api/v1/employees';

  list() {
    return this.http.get<Employee[]>(this.baseUrl);
  }

  create(body: CreateEmployeeRequest) {
    return this.http.post<Employee>(this.baseUrl, body);
  }
}
