from podaac_py import Podaac
import asyncio

podaac = Podaac()

async def main():
	print("hey")
	variables_info = await podaac.dataset_variables(id="PODAAC-ASOP2-25X01")
	print(variables_info)

	print("hey, it worked!")

asyncio.run(main())
