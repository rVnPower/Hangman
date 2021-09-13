word = 'Sech'
lifes = 7
guesses = []
false = []
done = False
print(f"""
            _________
            |       {'|' if lifes == 6 else ' '}
            |       {'0' if lifes == 5 else ' '}
            |
            |
            |
        __________
""")
while not done:
    def display(lifes):
        slash = '\\'
        print(f"""
            _________
            |       {'|' if lifes <= 6 else ' '}
            |       {'0' if lifes <= 5 else ' '}
            |      {'/' if lifes <= 3 else ' '}{'|' if lifes <= 4 else ' '}{slash if lifes <= 2 else ''}
            |      {'/' if lifes <= 1 else ' '}{' ' + slash if lifes == 0 else ''}
            |
        __________
         """)
    for letter in word:
        if letter.lower() in guesses:
            print(letter, end=' ')
        else:
            print('_', end=' ')

    guess = str(input())
    guesses.append(guess.lower())

    if guess.lower() not in word.lower():
        if guess.lower() in false:
            print('You have already guess this letter.')
            display(lifes)
            continue
        else:
            lifes -= 1
            display(lifes)
        false.append(guess)
        if lifes == 0:
            break

    done = True
    for letter in word:
        if letter.lower() not in guesses:
            done = False
    if guess.lower() == word:
        done = True
else:
    print('You won!')
    exit()
print(f'You lose. It was {word}.')
