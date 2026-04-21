import { Badge } from "./ui/badge"
import { Button } from "./ui/button"
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "./ui/card"

interface CardProps {
  title: string;
  description: string;
  image: string;
  link?: string;
  social?: string;
}

export function CardImage({ title, description, image, link, social }: CardProps) {

  return (
    <Card className="relative mx-auto w-full max-w-lg pt-0">
      <div className="absolute inset-0 z-30 aspect-video bg-black/35" />
      <img
        src={image}
        alt={title}
        className="relative z-20 aspect-video w-full object-cover brightness-60 grayscale dark:brightness-40"
      />
      <CardHeader>
        <Badge variant="secondary" className={`w-fit mb-2 border-2 ${social ? '' : 'hidden'} ${
          social?.toLowerCase() === 'youtube' ? 'border-red-600' :
          social?.toLowerCase() === 'tiktok' ? 'border-black dark:border-white' :
          social?.toLowerCase() === 'instagram' ? 'border-pink-600' :
          social?.toLowerCase() === 'pinterest' ? 'border-red-700' :
          'border-gray-300'
        }`}>
          {social || 'Featured'}
        </Badge>
        <CardTitle>{title}</CardTitle>
        <CardDescription>
          {description}
        </CardDescription>
      </CardHeader>
      <CardFooter>
        <Button className="w-full" asChild>
          <a href={link} target="_blank">View Video</a>
        </Button>
      </CardFooter>
    </Card>
  )
}
