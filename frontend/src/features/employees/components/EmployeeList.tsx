import { useEffect, useState } from 'react'
import { Button } from '@/components/Button'
import {
  Dialog,
  DialogHeader,
  DialogTitle,
  DialogClose,
  DialogContent,
  DialogFooter,
} from '@/components/Dialog'
import { Input } from '@/components/Input'
import { Label } from '@/components/Label'
import * as api from '../api'
import type { Employee, WorkLog } from '../data/model'

export function EmployeeList() {
  const [employees, setEmployees] = useState<Employee[]>([])
  const [selected, setSelected] = useState<Employee | null>(null)
  const [workLogs, setWorkLogs] = useState<WorkLog[]>([])
  const [showForm, setShowForm] = useState(false)
  const [showLogForm, setShowLogForm] = useState(false)
  const [newName, setNewName] = useState('')
  const [logDate, setLogDate] = useState('')
  const [logHours, setLogHours] = useState(0)

  useEffect(() => {
    loadEmployees()
  }, [])

  function loadEmployees() {
    api.listEmployees().then(setEmployees)
  }

  function loadWorkLogs(emp: Employee) {
    setSelected(emp)
    api.listWorkLogs(emp.id).then(setWorkLogs)
  }

  async function handleCreate() {
    if (!newName.trim()) return
    await api.createEmployee({ name: newName.trim() })
    setNewName('')
    setShowForm(false)
    loadEmployees()
  }

  async function handleLogHours() {
    if (!selected || !logDate || logHours <= 0) return
    await api.createWorkLog({ employee_id: selected.id, worked_on: logDate, hours: logHours })
    setLogDate('')
    setLogHours(0)
    setShowLogForm(false)
    loadWorkLogs(selected)
  }

  return (
    <div className="min-h-dvh">
      <header className="flex items-center justify-between bg-blue-600 px-6 py-3 text-white">
        <h1 className="text-lg font-semibold">Kora — Employees</h1>
        <Button variant="secondary" size="sm" onClick={() => setShowForm(true)}>
          Add Employee
        </Button>
      </header>

      <div className="p-6">
        <table className="w-full border-collapse">
          <thead>
            <tr className="border-b text-left text-sm font-medium text-muted-foreground">
              <th className="pb-2">Name</th>
              <th className="pb-2">Active</th>
              <th className="pb-2" />
            </tr>
          </thead>
          <tbody>
            {employees.map((e) => (
              <tr key={e.id} className="border-b last:border-0">
                <td className="py-2">{e.name}</td>
                <td className="py-2">{e.active ? 'Yes' : 'No'}</td>
                <td className="py-2 text-right">
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => {
                      setShowLogForm(true)
                      setSelected(e)
                    }}
                  >
                    Log Hours
                  </Button>
                  <Button variant="ghost" size="sm" onClick={() => loadWorkLogs(e)}>
                    View Logs
                  </Button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>

        {selected && (
          <section className="mt-8 border-t pt-6">
            <h2 className="mb-4 text-lg font-semibold">Work Logs — {selected.name}</h2>
            {workLogs.length === 0 ? (
              <p className="text-sm text-muted-foreground">No work logs yet.</p>
            ) : (
              <table className="w-full border-collapse">
                <thead>
                  <tr className="border-b text-left text-sm font-medium text-muted-foreground">
                    <th className="pb-2">Date</th>
                    <th className="pb-2">Hours</th>
                  </tr>
                </thead>
                <tbody>
                  {workLogs.map((w) => (
                    <tr key={w.id} className="border-b last:border-0">
                      <td className="py-2">{w.worked_on}</td>
                      <td className="py-2">{w.hours}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </section>
        )}
      </div>

      {/* Create Employee Dialog */}
      <Dialog open={showForm} onClose={() => setShowForm(false)}>
        <DialogHeader>
          <DialogTitle>New Employee</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <DialogContent>
          <div>
            <Label htmlFor="name">Name</Label>
            <Input
              id="name"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              placeholder="Employee name"
            />
          </div>
        </DialogContent>
        <DialogFooter>
          <Button variant="outline" onClick={() => setShowForm(false)}>
            Cancel
          </Button>
          <Button disabled={!newName.trim()} onClick={handleCreate}>
            Save
          </Button>
        </DialogFooter>
      </Dialog>

      {/* Log Hours Dialog */}
      <Dialog open={showLogForm} onClose={() => setShowLogForm(false)}>
        <DialogHeader>
          <DialogTitle>Log Hours</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <DialogContent>
          <div>
            <Label htmlFor="date">Date</Label>
            <Input
              id="date"
              type="date"
              value={logDate}
              onChange={(e) => setLogDate(e.target.value)}
            />
          </div>
          <div>
            <Label htmlFor="hours">Hours</Label>
            <Input
              id="hours"
              type="number"
              step="0.5"
              min="0"
              value={logHours}
              onChange={(e) => setLogHours(Number(e.target.value))}
            />
          </div>
        </DialogContent>
        <DialogFooter>
          <Button variant="outline" onClick={() => setShowLogForm(false)}>
            Cancel
          </Button>
          <Button disabled={!logDate || logHours <= 0} onClick={handleLogHours}>
            Save
          </Button>
        </DialogFooter>
      </Dialog>
    </div>
  )
}
