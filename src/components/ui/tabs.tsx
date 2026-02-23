import * as React from "react"
import { Tabs as BaseTabs } from "@base-ui/react"
import { cn } from "../../lib/utils"

export const Tabs = BaseTabs.Root

export const TabsList = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <BaseTabs.List
      ref={ref}
      className={cn(
        "inline-flex h-11 items-center justify-center rounded-2xl bg-slate-100/50 p-1.5 text-slate-500 backdrop-blur-sm dark:bg-slate-900/50 dark:text-slate-400",
        className
      )}
      {...props}
    />
  )
)
TabsList.displayName = "TabsList"

export const TabsTrigger = React.forwardRef<HTMLButtonElement, React.ButtonHTMLAttributes<HTMLButtonElement> & { value: string }>(
  ({ className, value, ...props }, ref) => (
    <BaseTabs.Tab
      ref={ref}
      value={value}
      className={cn(
        "inline-flex items-center justify-center whitespace-nowrap rounded-xl px-4 py-1.5 text-sm font-medium ring-offset-white transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500/20 disabled:pointer-events-none disabled:opacity-50 data-[selected]:bg-white data-[selected]:text-blue-600 data-[selected]:shadow-sm dark:ring-offset-slate-950 dark:data-[selected]:bg-slate-800 dark:data-[selected]:text-blue-400",
        className
      )}
      {...props}
    />
  )
)
TabsTrigger.displayName = "TabsTrigger"

export const TabsContent = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement> & { value: string }>(
  ({ className, value, ...props }, ref) => (
    <BaseTabs.Panel
      ref={ref}
      value={value}
      className={cn(
        "mt-4 focus-visible:outline-none",
        className
      )}
      {...props}
    />
  )
)
TabsContent.displayName = "TabsContent"
