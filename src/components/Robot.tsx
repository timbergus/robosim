import { Canvas } from '@react-three/fiber'
import { Join } from './Join'
import { Vector3 } from 'three'
import { get_robot } from '../pkg/robosim_lib'
import { ChangeEvent, useState } from 'react'
import { Slider } from './Slider'

export const Robot = () => {
  const [theta1, setTheta1] = useState(0.0)
  const [theta2, setTheta2] = useState(0.0)
  const [theta3, setTheta3] = useState(0.0)

  const robot = get_robot(theta1, theta2, theta3, 2, 3, 1)

  const positions: Vector3[] = [
    new Vector3(robot[0], robot[1], robot[2]),
    new Vector3(robot[4], robot[5], robot[6]),
    new Vector3(robot[8], robot[9], robot[10]),
    new Vector3(robot[12], robot[13], robot[14]),
  ]

  const handleAngleChange = (event: ChangeEvent<HTMLInputElement>) => {
    const {
      currentTarget: { name, value },
    } = event

    switch (name) {
      case 'theta1':
        setTheta1(Number(value))
        break
      case 'theta2':
        setTheta2(Number(value))
        break
      case 'theta3':
        setTheta3(Number(value))
        break
      default:
        break
    }
  }

  return (
    <>
      <div className="absolute p-11 grid gap-8 z-10">
        <Slider name="theta1" value={theta1} onChange={handleAngleChange} />
        <Slider name="theta2" value={theta2} onChange={handleAngleChange} />
        <Slider name="theta3" value={theta3} onChange={handleAngleChange} />
      </div>
      <Canvas style={{ width: '100vw', height: '100vh' }}>
        <ambientLight />
        <pointLight position={[10, 10, 10]} />
        {positions.map((position, index) => (
          <Join key={index} position={position} />
        ))}
      </Canvas>
    </>
  )
}
