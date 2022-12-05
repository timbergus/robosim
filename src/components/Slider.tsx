import { FC, HTMLProps } from 'react'

type SliderProps = HTMLProps<HTMLInputElement>

export const Slider: FC<SliderProps> = (props) => (
  <div className="grid grid-cols-2 gap-12 border p-4 rounded-lg">
    <span className="border text-center">{props.value} deg</span>
    <input type="range" min={-90} max={90} step={1} {...props} />
  </div>
)
