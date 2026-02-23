import * as React from "react"
import { Button as BaseButton } from "@base-ui/react"
import { cn } from "../../lib/utils"

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "outline" | "ghost" | "danger"
  size?: "sm" | "md" | "lg" | "icon"
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "primary", size = "md", ...props }, ref) => {
    const variants = {
      primary: "bg-blue-600 text-white hover:bg-blue-700 active:bg-blue-800",
      secondary: "bg-slate-200 text-slate-900 hover:bg-slate-300 active:bg-slate-400",
      outline: "border border-slate-300 bg-transparent hover:bg-slate-100 active:bg-slate-200",
      ghost: "bg-transparent hover:bg-slate-100 active:bg-slate-200",
      danger: "bg-red-600 text-white hover:bg-red-700 active:bg-red-800",
    }

    const sizes = {
      sm: "h-8 px-3 text-xs",
      md: "h-10 px-4 py-2 text-sm font-medium",
      lg: "h-12 px-6 text-base",
      icon: "h-10 w-10",
    }

    return (
      <BaseButton
        className={cn(
          "inline-flex items-center justify-center rounded-md transition-colors disabled:opacity-50 disabled:pointer-events-none focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
          variants[variant],
          sizes[size],
          className
        )}
        ref={ref}
        {...props}
      />
    )
  }
)
Button.displayName = "Button"

export { Button }
