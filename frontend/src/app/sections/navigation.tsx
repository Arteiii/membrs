import Image from "next/image";

export default function Navigation() {
    return (
        < nav className="fixed flex justify-between py-6 w-full lg:px-48 md:px-12 px-4 content-center bg-secondary z-10" >
            <div className="flex items-center" >
                <Image src='./assets/Logo_black.svg' alt="Logo" width={80} height={16} />
            </div>
            <ul className="font-montserrat items-center hidden md:flex">
                <li className="mx-3 ">
                    <a className="growing-underline" href="#HowItWorks">
                        How it works
                    </a>
                </li>
                <li className="growing-underline mx-3">
                    <a href="#Features">Features</a>
                </li>
                <li className="growing-underline mx-3">
                    <a href="#Pricing">Pricing</a>
                </li>
            </ul>
            <div className="font-montserrat hidden md:block">
                <button className="mr-6">Login</button>
                <button className="py-2 px-4 text-white bg-black rounded-3xl">
                    Signup
                </button>
            </div>
            <div id="showMenu" className="md:hidden">
                <Image src='./assets/logos/Menu.svg' width={20} height={16} alt="Menu icon" />
            </div>
            <div id='mobileNav' className="hidden px-4 py-6 fixed top-0 left-0 h-full w-full bg-secondary z-20 animate-fade-in-down">
                <div id="hideMenu" className="flex justify-end">
                    <Image src='./assets/logos/Cross.svg' alt="" height={64} width={64} />
                </div>
                <ul className="font-montserrat flex flex-col mx-8 my-24 items-center text-3xl">
                    <li className="my-6">
                        <a href="howitworks">How it works</a>
                    </li>
                    <li className="my-6">
                        <a href="features">Features</a>
                    </li>
                    <li className="my-6">
                        <a href="pricing">Pricing</a>
                    </li>
                </ul>
            </div>
        </nav >
    );
}
