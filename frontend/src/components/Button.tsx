/**
 * Button — Kora's primary action primitive.
 *
 * Wraps the underlying UI library (shadcn) with Kora's
 * design decisions: default size, icon spacing, radii, etc.
 *
 * Feature code MUST NOT import from `./ui/button` directly.
 */
import { forwardRef } from 'react'
import { Button as ShadcnButton, type ButtonProps as ShadcnButtonProps } from './ui/button'

export interface ButtonProps extends Omit<ShadcnButtonProps, 'size'> {
  size?: 'sm' | 'md' | 'lg' | 'icon'
}

const sizeMap = {
  sm: 'sm' as const,
  md: 'default' as const,
  lg: 'lg' as const,
  icon: 'icon' as const,
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ size = 'md', className, ...props }, ref) => (
    <ShadcnButton ref={ref} size={sizeMap[size]} className={className} {...props} />
  ),
)
Button.displayName = 'Button'

export { Button }
