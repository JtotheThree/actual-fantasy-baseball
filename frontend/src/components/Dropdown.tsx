import { ReactChildren, ReactNode } from "react"

type DropdownProps = {
  selected: any,
  key?: string,
  className?: string,
  id?: string,
  children: ReactNode,
}

export function Dropdown(props: DropdownProps) {
  return (
    <li className={props.className + " group relative dropdown"}>
      { props.selected }
      <div className="group-hover:block dropdown-menu absolute hidden h-auto py-3 flex-grow">
        <ul className="top-0 w-128 bg-paper shadow border-2 border-black px-6 py-2">
          { props.children }
        </ul>
      </div>
    </li>
  )
}

type DropdownItemProps = {
  onClick?: any,
  className?: string,
  id?: string,
  children?: ReactNode,
}

export function DropdownItem(props: DropdownItemProps) {
  const className = props.className + " text-left md:p-4 py-2 block font-bold hover:text-red-800";

  return (
    <li className="py-1">
      <button key={props.id} className={className} onClick={() => props.onClick ? props.onClick(props.id) : null}>
        {props.children}
      </button>
    </li>
  )
}