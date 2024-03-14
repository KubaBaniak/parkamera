export default function Header() {
  const menuItems = [{ text: "Home", url: "/" }, { text: "Get Camera Video", url: "/camera" }, { text: "Other", url: "/other" }]
  return (
    <header className="w-full h-16 bg-zinc-900">
      <ul className="h-full flex flex-row items-center gap-10 justify-end mr-20">
        {menuItems.map((item, index) => (
          <li key={`header-${index}`}>
            <a href={item.url}>{item.text}</a>
          </li>
        ))}
      </ul>
    </header >
  )
}

