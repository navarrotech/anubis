
let output = ""
let cache = ""

function solve(unsafeLines: string[], nextLine: string) {
  let content = ""
  let indexes = 0

  console.log({
    unsafeLines, nextLine
  })
  for (const unsafeLine of unsafeLines) {
    if (unsafeLine !== nextLine) {
      content += unsafeLine + '\n'
      indexes++
    }
    else {
      break
    }
  }

  return { content, indexes }
}

function main(content: string) {
  let out = ""

  content = content.replace(/\n$/, '').trim();

  const toWriteLines = content.split('\n')
  const cacheLines = cache.split('\n')
  const unsafeLines = output.replace(/\n$/, '').trim().split('\n')

  if (!cache || !output) {
    cache = content
    output = content
    return
  }

  let unsafeLineCounter = 0;
  let cacheLineCounter = 0;
  let toWriteCounter = 0;
  let allCounter = 0;

  while (true) {
    allCounter += 1;

    // Potentially user edited:
    const unsafeLine = unsafeLines[unsafeLineCounter]

    // Saved in cache past file:
    const cacheLine = cacheLines[cacheLineCounter]

    // What we want to write now
    const toWrite = toWriteLines[toWriteCounter]

    if (unsafeLine === undefined && cacheLine === undefined && toWrite === undefined) {
      break
    }

    // Safety
    if (allCounter > 30) {
      console.log('\n-- Infinite loop --\n')
      break
    }

    console.log({
      cacheLine, unsafeLine, toWrite
    })

    // If they're all matching...
    if (unsafeLine === cacheLine && unsafeLine === toWrite) {
      out += unsafeLine + '\n'
      unsafeLineCounter++
      cacheLineCounter++
      toWriteCounter++
      continue
    }

    // If the user has added a line
    if (toWrite === cacheLine && unsafeLine !== undefined) {
      let { content, indexes } = solve(
        unsafeLines.slice(unsafeLineCounter),
        cacheLines[cacheLineCounter + 1] || cacheLine,
      )
      console.log({ indexes, content })
      out += content
      unsafeLineCounter += indexes
      if (cacheLines[cacheLineCounter + 1] !== undefined) {
        cacheLineCounter++
        toWriteCounter++
      }
      continue
    }

    if (unsafeLine === undefined && cacheLine !== undefined && toWrite !== undefined) {
      out += toWrite + '\n'
      toWriteCounter++
      cacheLineCounter++
      continue
    }

    // If there's nothing else to do but add more writing stuff
    if (toWrite !== undefined && unsafeLine === undefined && cacheLine === undefined) {
      out += toWrite + '\n'
      toWriteCounter++
      continue
    }

    if (
      toWrite 
      && (unsafeLines[unsafeLineCounter + 1] === undefined || unsafeLine === undefined)
      && (cacheLines[cacheLineCounter + 1] === undefined || cacheLine === undefined)
    ) {
      out += toWrite + '\n'
      toWriteCounter++
      continue
    }
  }

  if (out.endsWith('\n\n')) {
    out = out.slice(0, -1)
  }

  if (!out.endsWith('\n')) {
    out += '\n'
  }

  cache = content
  output = out
}

// Should be able to add an index
function test1() {
  const firstChange = `a
b
c`
  
  const userChange = `a
b
b1
c
`
  
  const secondChange = `a
b
c
d`

  const expectedResult = `a
b
b1
c
d
`

  main(firstChange)
  output = userChange
  main(secondChange)

  console.log("\n >> Test 1 status: ", output === expectedResult)
  console.log('')
  if (output !== expectedResult) {
    console.log("Output:", output.split('\n').join(' '))
    console.log("Wanted:", expectedResult.split('\n').join(' '))
    console.log({ output, expectedResult })
  }
}

test1()
output = ""
cache = ""

// Should be able to change an index
function test2() {
  const firstChange = `apples
bananas
cats`
  
  const userChange = `apples
bananas and bats
cats
`
  
  const secondChange = `apples
bananas
cats
dogs`

  const expectedResult = `apples
bananas and bats
cats
dogs
`

  main(firstChange)
  output = userChange
  main(secondChange)

  console.log("\n >> Test 2 status: ", output === expectedResult)
  console.log('')
  if (output !== expectedResult) {
    console.log("Output:", output.split('\n').join(' '))
    console.log("Wanted:", expectedResult.split('\n').join(' '))
    console.log({ output, expectedResult })
  }
}

test2()
output = ""
cache = ""

// Should be able to change an index
function test3() {
  const firstChange = `apples
pears
bananas
oats
cats
dogs
airplanes`
  
  const userChange = `apples
pears
bananas fabulouso
oats
cats and bats
dogs
airplanes nippy
`
  
  const secondChange = `apples
pears
bananas
crackers
oats
cats
yankees
dogs
marxism?
airplanes`

  const expectedResult = `apples
pears
bananas fabulouso
crackers
oats
cats and bats
yankees
dogs
marxism?
airplanes nippy
`

  main(firstChange)
  output = userChange
  main(secondChange)

  console.log("\n >> Test 2 status: ", output === expectedResult)
  console.log('')
  if (output !== expectedResult) {
    console.log("Output:", output.split('\n').join(' '))
    console.log("Wanted:", expectedResult.split('\n').join(' '))
    console.log({ output, expectedResult })
  }
}

test3()
output = ""
cache = ""
