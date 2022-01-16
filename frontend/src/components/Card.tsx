import { Component, ReactChild, ReactNode } from "react"

type CardFooterProps = {
  children: ReactNode
}

export function CardFooterProps(props: CardFooterProps) {
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
    <div className="text-base text-gray-700">
      {props.children}
    </div>
  )
}

type CardSubHeaderProps = {
  content: string
}

export function CardSubHeader(props: CardSubHeaderProps) {
  return (
    <div className="mb-3 text-sm text-gray-500">
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

type CardProps = {
  children: ReactNode
}

export function Card(props: CardProps) {
  return (
    <div className="flex flex-col mx-auto shadow border-gray-700 w-full">
      {props.children}
    </div>
  )
}