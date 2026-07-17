import { createContext, useContext, type ReactNode } from 'react'
import { cn } from '@/lib/utils'
import { X } from 'lucide-react'

interface DialogContextType {
  open: boolean
  onClose: () => void
}

const DialogContext = createContext<DialogContextType | null>(null)

function useDialog() {
  const ctx = useContext(DialogContext)
  if (!ctx) throw new Error('Dialog components must be used within a Dialog')
  return ctx
}

export function Dialog({
  open,
  onClose,
  children,
}: {
  open: boolean
  onClose: () => void
  children: ReactNode
}) {
  if (!open) return null
  return (
    <DialogContext.Provider value={{ open, onClose }}>
      <div className="fixed inset-0 z-50 flex items-center justify-center">
        <div className="fixed inset-0 bg-black/50" onClick={onClose} />
        <div className="relative z-50 w-full max-w-lg rounded-lg border bg-background p-6 shadow-lg">
          {children}
        </div>
      </div>
    </DialogContext.Provider>
  )
}

export function DialogHeader({
  className,
  children,
}: {
  className?: string
  children: ReactNode
}) {
  return <div className={cn('mb-4 flex items-center justify-between', className)}>{children}</div>
}

export function DialogTitle({ children }: { children: ReactNode }) {
  return <h2 className="text-lg font-semibold">{children}</h2>
}

export function DialogClose() {
  const { onClose } = useDialog()
  return (
    <button onClick={onClose} className="rounded-sm opacity-70 hover:opacity-100">
      <X className="h-4 w-4" />
    </button>
  )
}

export function DialogContent({
  className,
  children,
}: {
  className?: string
  children: ReactNode
}) {
  return <div className={cn('space-y-4', className)}>{children}</div>
}

export function DialogFooter({
  className,
  children,
}: {
  className?: string
  children: ReactNode
}) {
  return (
    <div className={cn('mt-6 flex justify-end gap-2', className)}>{children}</div>
  )
}
