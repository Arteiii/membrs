import {GitHubLogoIcon} from "@radix-ui/react-icons";
import Link from 'next/link';


export default function Hero() {
    return (
        <section id="Hero"
                 className="pt-24 md:mt-0 md:h-screen flex flex-col justify-center text-center md:text-left md:flex-row md:justify-between md:items-center lg:px-48 md:px-12 px-4 bg-secondary">
            <div className="md:flex-1 md:mr-10">
                <h1 className="font-pt-serif text-5xl font-bold mb-7">
                    Connect, Manage, and Backup Your Discord
                    <span className="bg-underline1 bg-left-bottom bg-no-repeat pb-2 bg-100%">
                            &nbsp;membrs
                        </span>
                </h1>
                <p className="font-pt-serif font-normal mb-7">
                    membrs simplifies Discord community management by enabling seamless OAuth login for members,
                    facilitating efficient backup and essential operations.
                </p>
                <div className="font-montserrat flex">
                    <Link href="https://github.com/Arteiii/membrs" passHref legacyBehavior
                          className="flex items-center">
                        <a target="_blank">
                            <button
                                className="flex bg-black px-6 py-4 rounded-lg border-2 border-black border-solid text-white mr-2 mb-2">
                                <GitHubLogoIcon height={24} width={24} className="mr-3"></GitHubLogoIcon>
                                Checkout GitHub
                            </button>
                        </a>
                    </Link>
                    <Link href={`${process.env.NEXT_PUBLIC_URL}/api/oauth/url`} passHref legacyBehavior>
                        <a target="_blank">
                            <button
                                    className="px-6 py-4 border-2 border-black border-solid rounded-lg">
                                Login
                            </button>
                        </a>
                    </Link>
                </div>
            </div>
            <div className="flex justify-around md:block mt-8 md:mt-0 md:flex-1">
            </div>
        </section>
    )
        ;
}
