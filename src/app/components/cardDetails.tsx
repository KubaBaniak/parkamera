export default function CarDetails() {
  const carDetails = { timeArrived: new Date(), timeLeft: new Date() }
  return (
    <div className="w-40 flex flex-col items-center gap-2">
      <p className="font-bold text-center">Time arrived:</p>
      <p className="text-center">{carDetails.timeArrived.toUTCString()}</p>
    </div>
  )
}

