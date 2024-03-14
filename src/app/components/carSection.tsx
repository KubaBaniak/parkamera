import CarCard from "./carCard";

export default function CarSection() {
  const carSlots = [true, true, false, true, false, false];

  return (
    <div className="w-9/12 flex flex-col mt-9 flex-grow-0 gap-10 flex-wrap justify-center">
      {carSlots.map((isTaken, index) => (
        <CarCard key={`car-${index}`} isTaken={isTaken} />
      ))}
    </div>
  );
}
