# This file is of vega
# 
# vega is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# vega is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with vega. If not, see <https://www.gnu.org/licenses/>.


hardware() {
	subtitl "Hardware" 
	firstentry "MDL" && model "$soft"
	entry "CPU" && cpu "$soft"
	entry "GPU" && gpu "$soft"
	entry "MEM" && mem "$soft"
	entry "HDD" && hdd "$soft"
	lastentry "RES" && res "$soft"
}


software() {
	subtitl "Software"
	firstentry "OS " && os "$soft"
	entry "KER" && kernel "$soft"
	entry "PKG" && packages "$soft"
	entry "WMN" && wm "$soft"
	entry "TER" && terminal "$soft"
	entry "SHL" && shell "$soft"
	lastentry "IP4" && ip_local "$soft"
}


fetch() {
	title

	hardware
	printf '\n'

	software
	printf '\n'
}