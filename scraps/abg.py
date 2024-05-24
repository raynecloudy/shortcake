file = open(input("open a file..."), "r").read()

class abg:
	def interpret(file):
		try:
			lines = file.splitlines()

			if lines[0] == "1.0.1":
				brightnesslevels = ["1", "2", "3", "4", "x"]
				brightness = ["  ", "░░", "▓▓", "██", "\033[0m  \033[40m"]
				colornames = ["r", "g", "y", "v", "p", "b", "w", "x"]
				colors = ["\033[31m", "\033[32m", "\033[33m", "\033[34m", "\033[35m", "033[36m", "\033[37m", ""]
				result = "\033[40m"
				for y in range(len(lines)-2):
					linepixels = lines[y+2].split(" ")
					for x in range(len(linepixels)):
						pixel = linepixels[x]
						if pixel[0].lower() in colornames:
							result += colors[colornames.index(pixel[0].lower())]
							if pixel[1].lower() in brightnesslevels:
								result += brightness[brightnesslevels.index(pixel[1].lower())]
							else:
								raise ValueError("no such brightness level \"%s\"", pixel[1].lower())
						else:
							raise ValueError("no such color \"%s\"", pixel[0].lower())
					result += "\n"
				return result
			else:
				raise ValueError("no such version \"%s\"", lines[0])
		except AttributeError:
			return "something went wrong!"

	def version(file):
		return file.splitlines()[0]

print(abg.interpret(file))
