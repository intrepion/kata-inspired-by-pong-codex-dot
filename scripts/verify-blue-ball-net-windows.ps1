param(
    [Parameter(Mandatory = $true)]
    [string]$Capture
)

$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$image = [System.Drawing.Bitmap]::new((Resolve-Path -LiteralPath $Capture).Path)
try {
    if ($image.Width -ne 1024 -or $image.Height -ne 640) {
        throw "Expected the 1024x640 Blue Engine headless tick-240 capture."
    }
    $notWhite = 0
    for ($y = 322; $y -le 328; $y++) {
        for ($x = 511; $x -le 512; $x++) {
            $pixel = $image.GetPixel($x, $y)
            if ($pixel.R -ne 255 -or $pixel.G -ne 255 -or $pixel.B -ne 255) {
                $notWhite++
            }
        }
    }
    if ($notWhite -ne 0) {
        throw "Ball is obscured at $notWhite of 14 net-overlap pixels."
    }
    Write-Output 'All 14 net-overlap pixels show the ball.'
} finally {
    $image.Dispose()
}
