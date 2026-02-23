import * as React from "react"
import { ChevronDown } from "lucide-react"
import { cn } from "../../lib/utils"

export interface SelectProps extends React.SelectHTMLAttributes<HTMLSelectElement> {
  options: { label: string; value: string }[]
}

const Select = React.forwardRef<HTMLSelectElement, SelectProps>(
  ({ className, options, ...props }, ref) => {
    return (
      <div className="relative inline-block w-full">
        <select
          ref={ref}
          className={cn(
            "flex h-10 w-full appearance-none rounded-xl border border-slate-200 bg-white/50 px-3.5 py-2 text-sm text-slate-900 shadow-sm backdrop-blur-sm transition-all focus:outline-none focus:ring-2 focus:ring-blue-500/10 focus:border-blue-500/50 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-slate-900/50 dark:border-slate-800 dark:text-white dark:focus:ring-blue-600/20 dark:focus:border-blue-600/50",
            className
          )}
          {...props}
        >
          {options.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
        <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-slate-500">
          <ChevronDown className="h-4 w-4" />
        </div>
      </div>
    )
  }
)
Select.displayName = "Select"

export { Select }
