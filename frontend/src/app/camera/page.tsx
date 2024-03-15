import Header from "../components/header";
import Image from 'next/image'

export default function CameraFootage() {
  return (
    <main className="flex min-h-screen flex-col items-center bg-gradient-to-r from-indigo-500 to-purple-500">
      <Header />
      <div className="flex flex-col justify-center flex-grow" >
        <Image
          src="/camera.jpg"
          width={700}
          height={700}
          loading="lazy"
          alt=""
        />
      </div>
    </main>
  )
}
