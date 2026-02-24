import * as React from "react"
import { Button as BaseButton } from "@base-ui/react"
import { cn } from "../../lib/utils"

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "outline" | "ghost" | "danger" | "success"
  size?: "sm" | "md" | "lg" | "icon"
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "primary", size = "md", ...props }, ref) => {
    const variants = {
      primary: "bg-blue-600 text-white hover:bg-blue-700 active:bg-blue-800 shadow-sm shadow-blue-500/10",
      secondary: "bg-slate-50 text-slate-600 hover:bg-slate-100 active:bg-slate-200",
      outline: "border border-slate-100 bg-white hover:bg-slate-50 text-slate-600 shadow-sm shadow-slate-200/5",
      ghost: "bg-transparent hover:bg-slate-50 text-slate-500 hover:text-slate-700",
      danger: "bg-red-500 text-white hover:bg-red-600 active:bg-red-700 shadow-sm shadow-red-500/10",
      success: "bg-emerald-500 text-white hover:bg-emerald-600 active:bg-emerald-700 shadow-sm shadow-emerald-500/10",
    }

    const sizes = {
      sm: "h-7 px-2.5 text-[10px] font-bold uppercase tracking-tight",
      md: "h-9 px-3.5 text-xs font-semibold",
      lg: "h-11 px-5 text-sm font-semibold",
      icon: "h-8 w-8",
    }

    return (
      <BaseButton
        className={cn(
          "inline-flex items-center justify-center rounded-xl transition-all duration-200 disabled:opacity-50 disabled:pointer-events-none focus:outline-none focus:ring-2 focus:ring-blue-500/10 active:scale-[0.98]",
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
