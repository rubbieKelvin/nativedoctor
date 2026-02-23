import * as React from "react"
import { Tabs as BaseTabs } from "@base-ui/react"
import { cn } from "../../lib/utils"

export const Tabs = BaseTabs.Root

export const TabsList = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <BaseTabs.List
      ref={ref}
      className={cn(
        "inline-flex h-10 items-center justify-center rounded-md bg-slate-100 p-1 text-slate-500 dark:bg-slate-800 dark:text-slate-400",
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
        "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-white transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[selected]:bg-white data-[selected]:text-slate-950 data-[selected]:shadow-sm dark:ring-offset-slate-950 dark:focus-visible:ring-slate-300 dark:data-[selected]:bg-slate-950 dark:data-[selected]:text-slate-50",
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
        "mt-2 ring-offset-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950 focus-visible:ring-offset-2 dark:ring-offset-slate-950 dark:focus-visible:ring-slate-300",
        className
      )}
      {...props}
    />
  )
)
TabsContent.displayName = "TabsContent"
