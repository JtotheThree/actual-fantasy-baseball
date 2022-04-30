import { ReactNode } from "react"

type HoverInfoProps = {
  text: string
  children: ReactNode
}

export function HoverInfo(props: HoverInfoProps) {
  return (
    <div className="group relative px-4">
      <span>{props.text}</span>
      <div className="group-hover:block absolute hidden h-auto bg-paper z-50">
        {props.children}
      </div>
    </div>
  )
}
