import { Cylinder } from '@react-three/drei'
import { ThreeElements } from '@react-three/fiber'

export const Link = (props: ThreeElements['mesh']) => (
  <Cylinder {...props} args={[0.1, 0.1, 2]}>
    <meshBasicMaterial color="green" />
  </Cylinder>
)
