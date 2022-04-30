import { ReactNode } from "react"
import {Link} from "react-router-dom"

type CardFooterProps = {
  children: ReactNode
}

export function CardFooter(props: CardFooterProps) {
  return (
    <div className="mx-4 mt-2 mb-4">
      {props.children}
    </div>
  )
}

type CardBodyProps = {
  children: ReactNode
}

export function CardBody(props: CardBodyProps) {
  return (
    <div className="p-4 text-base text-gray-700 bg-paper">
      {props.children}
    </div>
  )
}

type CardSubHeaderProps = {
  content: string
}

export function CardSubHeader(props: CardSubHeaderProps) {
  return (
    <div className="p-2 mb-3 text-sm text-gray-500 bg-paper">
      {props.content}
    </div>
  )
}

type CardHeaderProps = {
  title: string
}

export function CardHeader(props: CardHeaderProps) {
  return (
    <div className="py-1 text-xl font-bold text-center bg-gray-700 text-paper">
      {props.title}
    </div>
  )
}

type CardHeaderLinkProps = {
  title: string
  to: string
}

export function CardHeaderLink(props: CardHeaderLinkProps) {
  return (
    <div className="py-1 text-xl font-bold text-center bg-gray-700 text-paper">
      <Link to={props.to}>{props.title}</Link>
    </div>
  )
}

type CardProps = {
  children: ReactNode
}

export function Card(props: CardProps) {
  return (
    <div className="flex flex-col mx-auto shadow border-gray-700 bg-paper/100 w-full">
      {props.children}
    </div>
  )
}
