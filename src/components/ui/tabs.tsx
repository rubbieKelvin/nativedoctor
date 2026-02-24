import * as React from "react"
import { Tabs as BaseTabs } from "@base-ui/react"
import { cn } from "../../lib/utils"

export const Tabs = BaseTabs.Root

export const TabsList = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <BaseTabs.List
      ref={ref}
      className={cn(
        "inline-flex h-9 items-center justify-center rounded-xl bg-slate-100/30 p-1 text-slate-500 backdrop-blur-sm",
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
        "inline-flex items-center justify-center whitespace-nowrap rounded-lg px-3 py-1 text-xs font-bold tracking-tight ring-offset-white transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500/10 disabled:pointer-events-none disabled:opacity-50 data-[selected]:bg-white data-[selected]:text-blue-600 data-[selected]:shadow-sm",
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
