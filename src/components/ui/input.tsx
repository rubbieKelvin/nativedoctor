import * as React from "react"
import { Input as BaseInput } from "@base-ui/react"
import { cn } from "../../lib/utils"

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, ...props }, ref) => {
    return (
      <BaseInput
        className={cn(
          "flex h-10 w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-slate-900 dark:border-slate-700 dark:text-white dark:placeholder:text-slate-500 dark:focus:ring-blue-600 dark:focus:border-blue-600",
          className
        )}
        ref={ref}
        {...props}
      />
    )
  }
)
Input.displayName = "Input"

export { Input }
