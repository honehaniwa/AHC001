def main():
    with open('out') as f:
        lines = f.readlines()
    n = int(lines.pop(0).strip())
    print(n, len(lines) // n)
    for i in range(0, len(lines) // n):
        with open('./trans_output/{}.txt'.format(i), mode='w') as f:
            for l in lines[i*n:i*n+n]:
                f.write(l)



if __name__ == '__main__':
    main()