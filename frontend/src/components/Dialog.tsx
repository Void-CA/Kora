/**
 * Dialog — Kora's modal primitive.
 *
 * Imposes consistent padding, title sizing, backdrop blur, and
 * close-button placement across all dialogs in Kora.
 */
import { type ReactNode } from 'react'
import { cn } from '@/lib/utils'
import {
  Dialog as ShadcnDialog,
  DialogHeader as ShadcnHeader,
  DialogTitle,
  DialogClose,
  DialogContent as ShadcnContent,
  DialogFooter as ShadcnFooter,
} from './ui/dialog'

export function Dialog(props: {
  open: boolean
  onClose: () => void
  children: ReactNode
}) {
  return <ShadcnDialog {...props} />
}

export function DialogHeader({
  className,
  children,
  ...props
}: {
  className?: string
  children: ReactNode
}) {
  return (
    <ShadcnHeader className={cn('border-b pb-3', className)} {...props}>
      {children}
    </ShadcnHeader>
  )
}

export { DialogTitle, DialogClose }

export function DialogContent({
  className,
  children,
  ...props
}: {
  className?: string
  children: ReactNode
}) {
  return (
    <ShadcnContent className={cn('space-y-5', className)} {...props}>
      {children}
    </ShadcnContent>
  )
}

export function DialogFooter({
  className,
  children,
  ...props
}: {
  className?: string
  children: ReactNode
}) {
  return (
    <ShadcnFooter className={cn('border-t pt-4', className)} {...props}>
      {children}
    </ShadcnFooter>
  )
}
