import React, { MutableRefObject, Ref } from 'react';

namespace Form {
  type NumberProps = {
    label: string,
    state: React.Dispatch<React.SetStateAction<number>>
    min: number,
    max: number,
    default: number,
  }

  export function Number(props: NumberProps) {
    return (
      <div className="flex flex-col py-3">
        <input
          className="p-2 border-0 border-b-2 border-black outline-none bg-paper focus:ring focus:ring-red-700"
          type="number"
          min={props.min}
          max={props.max}
          value={props.default}
          onChange={(e) => props.state(parseInt(e.target.value))}
        />
        <label className="px-4 pb-2 font-semibold">
          { props.label }
        </label>
      </div>
    )
  }

  type CheckboxProps = {
    label: string,
    state: React.Dispatch<React.SetStateAction<boolean>>,
  }

  export function Checkbox(props: CheckboxProps) {
    return (
      <div className="flex flex-col py-3">
        <input
          className="form-checkbox h-5 w-5 text-red-700"
          type="checkbox"
          onChange={(e) => props.state(e.target.checked)}
        />
        <label className="px-4 pb-2 font-semibold">
          { props.label }
        </label>
      </div>
    )
  }

  type InputProps = {
    label: string,
    type?: string,
    state: React.Dispatch<React.SetStateAction<string>>,
    disabled?: boolean
  }

  export function Input(props: InputProps) {
    return (
      <div className="flex flex-col py-3">
          <input
            className="p-2 border-0 border-b-2 border-black outline-none bg-paper focus:ring focus:ring-red-700 disabled:text-transparent disabled:border-gray-400"
            onChange={(e) => props.state(e.target.value)}
            type={props.type ? props.type : "input"}
            disabled={props.disabled}
          />
          {props.disabled
          ? <label className="px-4 pb-2 font-semibold text-gray-400">
              {props.label}
          </label>
          : <label className="px-4 pb-2 font-semibold">
              {props.label}
          </label>
          }
      </div>
    )
  }

  type TitleProps = {
    title: string
  }

  function Title(props: TitleProps) {
    return (
      <h2 className="h-5 text-center border-b-2 border-gray-700 tracking-wid px-4 text-3xl font-title">
        <span className="bg-paper px-8">
          {props.title}
        </span>
      </h2>
    )
  }

  type FullPageProps = {
    error: string,
    title: string,
    children: React.ReactNode,
    submitLabel: string,
    onSubmit: React.FormEventHandler<HTMLFormElement>,
  }

  export function FullPage(props: FullPageProps) {
    return (
      <div className="w-full justify-self-center md:w-1/2 mx-auto flex flex-col flex-wrap justify-between p-16">
        <Title title={props.title} />
        <form className="md:w-1/2-screen m-0 p-12 w-full tw-h-full shadow-md" onSubmit={props.onSubmit}>
          <span className="text-red-800">{props.error}</span>
          {props.children}

          <div className="mt-2">
            <button
              className="btn p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800"
              type="submit"
            >
              {props.submitLabel}
            </button>
          </div>
        </form>
      </div>
    )
  }
}

export default Form;