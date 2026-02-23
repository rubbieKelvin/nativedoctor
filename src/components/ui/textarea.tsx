import * as React from "react"
import { cn } from "../../lib/utils"

export interface TextareaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {}

const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ className, ...props }, ref) => {
    return (
      <textarea
        className={cn(
          "flex min-h-[100px] w-full rounded-xl border border-slate-200 bg-white/50 px-3.5 py-2 text-sm text-slate-900 shadow-sm backdrop-blur-sm transition-all placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500/10 focus:border-blue-500/50 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-slate-900/50 dark:border-slate-800 dark:text-white dark:placeholder:text-slate-500 dark:focus:ring-blue-600/20 dark:focus:border-blue-600/50",
          className
        )}
        ref={ref}
        {...props}
      />
    )
  }
)
Textarea.displayName = "Textarea"

export { Textarea }
