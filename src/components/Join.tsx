import { ThreeElements } from '@react-three/fiber'

export const Join = (props: ThreeElements['mesh']) => (
  <mesh {...props}>
    <sphereGeometry args={[0.3]} />
    <meshStandardMaterial color="blue" />
  </mesh>
)
